# Shaders

## Quad vertices

Counter-clockwise order, starting from the `BottomLeft` corner.

There are two ways to define two triangles (split along / diagonal, **BottomLeft-TopRight** shared edge):

```text
     TopLeft               TopRight
  [-0.5,0.5]       +       [0.5, 0.5]
  [0.0, 1.0]       Y       [1.0, 1.0]
            ┌─────────────┐ 
            │             │
            │             │
        - X │  [0.0,0.0]  │ X +
            │             │
            │             │
            └─────────────┘ 
  [0.0, 0.0]       Y       [1.0, 0.0]
 [-0.5,-0.5]       -       [0.5,-0.5]
  BottomLeft               BottomRight
```

- **Full 6 vertices**:

`Indexes = [0, 1, 2, 3, 4, 5]`

| Index | Vertex      |
| ----- | ----------- |
| 0     | BottomLeft  |
| 1     | TopRight    |
| 2     | TopLeft     |
| 3     | BottomLeft  |
| 4     | BottomRight |
| 5     | TopRight    |

| Triangle | Indexes   | Ordered                             | Area       |
| -------- | --------- | ----------------------------------- | ---------- |
| 0        | 0 → 1 → 2 | BottomLeft → TopRight → TopLeft     | UpperLeft  |
| 1        | 3 → 4 → 5 | BottomLeft → BottomRight → TopRight | LowerRight |

- **Short 4 vertices**:

`Indexes = [0, 2, 3, 0, 2, 1]`

| Index | Vertex      |
| ----- | ----------- |
| 0     | BottomLeft  |
| 1     | BottomRight |
| 2     | TopRight    |
| 3     | TopLeft     |

| Triangle | Indexes   | Ordered                             | Area       |
| -------- | --------- | ----------------------------------- | ---------- |
| 0        | 0 → 2 → 3 | BottomLeft → TopRight → TopLeft     | UpperLeft  |
| 1        | 0 → 2 → 1 | BottomLeft → BottomRight → TopRight | LowerRight |

## Quad grid: `vec4<T>`

```text
                  0
   0 TopLeft     Top     TopRight 1
            ┌───────────┐ 
            │           │
     3 Left │           │ Right 1
            │           │
            └───────────┘ 
  BottomLeft    Bottom   BottomRight
           3      2      2
```

Calculates the indexes of `1x4` from `2x2`.

`1x4`
| Index | Indice | Edge   | Corner       | Inset (h, v) |
| ----- | ------ | ------ | ------------ | ------------ |
| 0     | x      | Top    | TopLeft      | [w,x]        |
| 1     | y      | Right  | TopRight     | [y,x]        |
| 2     | z      | Bottom | BottomRight  | [y,z]        |
| 3     | w      | Left   | BottomLeft   | [w,z]        |

`2x2`
| Row / Column |  0  |  1  |    
| ------------ | --- | --- |    
| 0            |  0  |  1  |    
| 1            |  2  |  0  |    

### Edges

`[Top, Right, Bottom, Left]`

Data:

- `border_widths`: `vec4<f32>`

### Corners

`[TopLeft, TopRight, BottomRight, BottomLeft]`

Data:

- `corner_radii`: `vec4<f32>`

## Rounded Boxes

- [Dist Functions](https://iquilezles.org/articles/distfunctions/)
- [Rounded Boxes](https://iquilezles.org/articles/roundedboxes/)

## Box Shadows

- [Fast Rounded Rectangle Shadows](https://madebyevan.com/shaders/fast-rounded-rectangle-shadows/)

- [Fast Shadows on Rectangles](https://stereopsis.com/shadowrect/)

- [Thousands of Styled Rectangles in 120FPS on GPU](https://tchayen.com/thousands-styled-rectangles-in-120fps-on-gpu)

- [How to draw styled rectangles using the GPU and Metal](https://www.warp.dev/blog/how-to-draw-styled-rectangles-using-the-gpu-and-metal)

- [Blurred rounded rectangles](https://raphlinus.github.io/graphics/2020/04/21/blurred-rounded-rects.html)
