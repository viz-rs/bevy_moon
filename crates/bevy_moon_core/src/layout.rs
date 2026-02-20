use std::fmt;

use bevy_ecs::{
    entity::{Entity, EntityHashMap},
    error::Result,
    resource::Resource,
    system::Query,
};
use bevy_math::{UVec2, Vec2};
use bevy_platform::collections::hash_map::Entry;
use bevy_text::{ComputedTextBlock, FontCx};
use stacksafe::stacksafe;
use taffy::{Layout, NodeId, Style, TaffyTree};

use crate::measure::{MeasureArgs, NodeContext};

#[derive(Resource)]
pub struct UiLayoutTree {
    taffy: TaffyTree<NodeContext>,
    node_map: EntityHashMap<NodeId>,
}

impl fmt::Debug for UiLayoutTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LayoutTree")
            .field("taffy", &self.taffy)
            .field("node_map", &self.node_map)
            .finish()
    }
}

impl Default for UiLayoutTree {
    fn default() -> Self {
        Self {
            taffy: TaffyTree::new(),
            node_map: EntityHashMap::new(),
        }
    }
}

unsafe impl Send for UiLayoutTree {}
unsafe impl Sync for UiLayoutTree {}

fn _assert_send_sync_layout_tree_impl_safe() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<EntityHashMap<NodeId>>();
    _assert_send_sync::<UiLayoutTree>();
}

const EXPECT_MESSAGE: &str = "we should avoid taffy layout errors by construction if possible";

impl UiLayoutTree {
    pub fn upsert_node(
        &mut self,
        entity: Entity,
        style: Style,
        node_context: Option<NodeContext>,
    ) -> NodeId {
        let taffy = &mut self.taffy;

        let node_id = match self.node_map.entry(entity) {
            Entry::Occupied(entry) => {
                let node_id = *entry.get();
                self.set_node_style(node_id, style);
                self.set_node_context(node_id, node_context);
                node_id
            }
            Entry::Vacant(entry) => {
                let node_id = if let Some(context) = node_context {
                    taffy.new_leaf_with_context(style, context)
                } else {
                    taffy.new_leaf(style)
                }
                .expect(EXPECT_MESSAGE);
                entry.insert(node_id);
                node_id
            }
        };

        node_id
    }

    pub fn set_node_style(&mut self, id: NodeId, style: Style) {
        self.taffy.set_style(id, style).expect(EXPECT_MESSAGE);
    }

    pub fn set_node_context(&mut self, id: NodeId, node_context: Option<NodeContext>) {
        self.taffy
            .set_node_context(id, node_context)
            .expect(EXPECT_MESSAGE);
    }

    pub fn set_node_children(&mut self, node_id: NodeId, children: &[NodeId]) {
        self.taffy
            .set_children(node_id, children)
            .expect(EXPECT_MESSAGE);
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.node_map.contains_key(&entity)
    }

    pub fn remove_nodes(&mut self, entities: impl Iterator<Item = Entity>) {
        for entity in entities {
            self.remove_node(entity);
        }
    }

    pub fn remove_node(&mut self, entity: Entity) {
        let Some(node_id) = self.node_map.remove(&entity) else {
            return;
        };
        self.taffy.remove(node_id).expect(EXPECT_MESSAGE);
    }

    pub fn remove_node_children(&mut self, entity: Entity) {
        let Some(&node_id) = self.node_map.get(&entity) else {
            return;
        };
        self.set_node_children(node_id, &[]);
    }

    pub fn remove_nodes_children(&mut self, entities: impl Iterator<Item = Entity>) {
        for entity in entities {
            self.remove_node_children(entity);
        }
    }

    pub fn get_layout(&self, entity: Entity) -> Result<Layout> {
        let Some(&node_id) = self.node_map.get(&entity) else {
            return Err("Invalid hierarchy".into());
        };

        let layout = self.taffy.layout(node_id)?;

        Ok(*layout)
    }

    #[stacksafe]
    pub fn compute_layout<'a>(
        &mut self,
        root_node_entity: Entity,
        physical_size: UVec2,
        _text_block_query: &'a mut Query<&mut ComputedTextBlock>,
        font_system: &'a mut FontCx,
    ) {
        let node_id = *self.node_map.entry(root_node_entity).or_insert_with(|| {
            let node_id = self.taffy.new_leaf(Style::DEFAULT).expect(EXPECT_MESSAGE);
            node_id
        });

        let physical_size = physical_size.as_vec2();

        let available_space = taffy::Size {
            width: taffy::AvailableSpace::Definite(physical_size.x),
            height: taffy::AvailableSpace::Definite(physical_size.y),
        };

        self.taffy
            .compute_layout_with_measure(
                node_id,
                available_space,
                |known_dimensions, available_space, _id, node_context, style| {
                    let Some(node_context) = node_context else {
                        return taffy::Size::ZERO;
                    };

                    // let text_buffer =
                    //     TextMeasur::needs_buffer(known_dimensions.height, available_space.width)
                    //         .then(|| node_context.get_text_buffer(text_block_query))
                    //         .flatten();
                    let text_buffer = None;

                    let args = MeasureArgs {
                        known_dimensions,
                        available_space,
                        font_system,
                        text_buffer,
                    };

                    let Vec2 { x, y } = node_context.measure(args, style);

                    taffy::Size {
                        width: x,
                        height: y,
                    }
                },
            )
            .expect(EXPECT_MESSAGE);
    }
}

// Debug and perf
#[allow(dead_code)]
impl UiLayoutTree {
    fn count_all_children(&self, parent: NodeId) -> Result<u32> {
        let mut count = 0;

        for child in self.taffy.children(parent)? {
            // Count this child.
            count += 1;

            // Count all of this child's children.
            count += self.count_all_children(child)?
        }

        Ok(count)
    }

    fn max_depth(&self, depth: u32, parent: NodeId) -> Result<u32> {
        use taffy::TraversePartialTree;

        println!(
            "{parent:?} at depth {depth} has {} children",
            self.taffy.child_count(parent)
        );

        let mut max_child_depth = 0;

        for child in self.taffy.children(parent)? {
            max_child_depth = std::cmp::max(max_child_depth, self.max_depth(0, child)?);
        }

        Ok(depth + 1 + max_child_depth)
    }

    fn get_edges(&self, parent: NodeId) -> Result<Vec<(NodeId, NodeId)>> {
        let mut edges = Vec::new();

        for child in self.taffy.children(parent)? {
            edges.push((parent, child));

            edges.extend(self.get_edges(child)?);
        }

        Ok(edges)
    }

    pub fn print_tree(&mut self) {
        for (entity, &node_id) in &self.node_map {
            println!("Entity: {entity}");
            self.taffy.print_tree(node_id);
            // self.get_edges(node_id);
        }
    }
}

#[cfg(test)]
mod tests {}
