/* Caring for a plant can be hard work, but since you tend to it regularly, you have a plant that grows consistently. Each day, its height increases by a fixed amount represented by the integer upSpeed. But due to lack of sunlight, the plant decreases in height every night, by an amount represented by downSpeed.

Since you grew the plant from a seed, it started at height 0 initially. Given an integer desiredHeight, your task is to find how many days it'll take for the plant to reach this height. */

fn growingPlant(u: i32, d: i32, h: i32) -> i32 {
    if u > h {1 } else {((h-d) as f32 / (u - d) as f32).ceil() as i32}
}