Normal_Heights
==============

Just a simple (rust) library to create a normal map from a height map.

Also includes a thin binary wrapper.

Example Library Usage
---------------------

```rust
    let img = image::open(input)?;
    let normal_map = normal_heights::map_normals(&img);
    normal_map.save(output).unwrap();
```

```rust
    let img = image::open(input)?;
    let strength = 3.14;
    let normal_map = normal_heights::map_normals_with_strength(&img, strength);
    normal_map.save(output).unwrap();
```

