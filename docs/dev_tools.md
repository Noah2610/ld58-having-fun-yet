# Having Fun with `dev_tools` yet?
## Compile and run with `dev_tools` enabled
```bash
cargo run --features dev_tools
```

## `dev_tools`-enabled features
| **Key**           | **Action**                                                                     |
| -----------------:|:------------------------------------------------------------------------------ |
| `F1`              | Toggle FPS display                                                             |
| `F2`              | Toggle all & primary inspectors (quality, camera)                              |
| `F3`              | Toggle collider hitboxes overlay, Toggle secondary inspectors (if F2 enabled)  |
| `F4`              | Toggle UI debug overlay                                                        |
| `Control+O`       | Pause everything except player movement, don't open pause UI                   |
| `Control+E`       | Toggle enemy behavior, wave spawning, and survival timer ticking               |
| `.` / `Control+.` | Add +10sec/+60sec to survival timer                                            |
| `,` / `Control+,` | Sub -10sec/-60sec from survival timer                                          |
| `Control+R`       | Reset survival timer                                                           |
| `Control+I`       | Toggle player invincibility                                                    |
| `R`               | Retrieve Ball

<!-- vim: conceallevel=0 -->
