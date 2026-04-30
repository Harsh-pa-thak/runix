// Layer 1: Common Library — 30+ hardcoded ASCII illustrations compiled into the binary.
// Lookup is a simple match on lowercased word. Zero latency, zero network.

pub fn lookup(word: &str) -> Option<&'static str> {
    match word.to_lowercase().as_str() {
        "cat" => Some(r#"
  /\_____/\
 (  o   o  )
 (  =^=   )
  (______) "#),

        "dog" => Some(r#"
  / \__
 (    @\___
 /         O
/   (_____/
/_____/   U "#),

        "skull" => Some(r#"
  ___
 /   \
| o o |
|  ^  |
|_____|
|_|_|_| "#),

        "sun" => Some(r#"
    \   |   /
  '-.       .-'
 ---( * * * )---
  .-'       '-.
    /   |   \ "#),

        "moon" => Some(r#"
  _..._
.' .   '.
|  o     \
'.    .   |
  '-.  _.' "#),

        "house" => Some(r#"
    /\
   /  \
  / /\ \
 /_/  \_\
|  _  _  |
|_|__|__|_| "#),

        "tree" => Some(r#"
    *
   ***
  *****
 *******
   | |
   | | "#),

        "sword" => Some(r#"
    |
   /|\
  / | \
 /  |  \
    |
   /|\
  / | \ "#),

        "fish" => Some(r#"
  ><(((o>
 ~~~~~~~~~  "#),

        "bird" => Some(r#"
   ___
  (o o)
  ( V )
--m-m-- "#),

        "car" => Some(r#"
  ___________
 /     |     \
|  O   |   O  |
 \_____|_____/
  o         o "#),

        "rocket" => Some(r#"
    /\
   /  \
  | [] |
  |    |
 /|    |\
/ |____| \
\  ____  /
 |_|  |_| "#),

        "heart" => Some(r#"
  _   _
 / \ / \
|   *   |
 \     /
  \   /
   \ /
    V "#),

        "star" => Some(r#"
    *
   ***
  *****
 *******
*********
   | |
   | | "#),

        "coffee" => Some(r#"
  ( (
   ) )
 ........
 |      |
 |      |
 '------' "#),

        "pizza" => Some(r#"
   ______
  /  /\  \
 /  /  \  \
/__/____\__\
\  \    /  /
 \  \  /  /
  \__\/  / "#),

        "crown" => Some(r#"
|  *  *  *  |
| *  *  *  *|
|  ________  |
| |________| | "#),

        "ghost" => Some(r#"
  _______
 /       \
|  o   o  |
|    ^    |
|  \___/  |
|___|_|___| "#),

        "robot" => Some(r#"
  ________
 |  o  o  |
 |  ____  |
 | |    | |
 |_|    |_|
  |      |
 /|      |\ "#),

        "snake" => Some(r#"
   __
  / _)
 .-^^^-/ /
__/       /
<__.|_|-|_| "#),

        "dragon" => Some(r#"
       __   _
     o( _'-' )
    (__> \ >_/
   (__) \(_)
   (__) \(_) "#),

        "wave" => Some(r#"
  ~~~~
 /    \    /
/      \  /
        \/ "#),

        "mountain" => Some(r#"
        /\
       /  \
      / /\ \
     / /  \ \
    /_/    \_\ "#),

        "flower" => Some(r#"
  @   @
 @@@  @@@
  @@@@@
   | |
   | |
  _|_|_ "#),

        "clock" => Some(r#"
  _____
 /     \
| 12    |
|  9  3 |
|   6   |
 \_____/ "#),

        "key" => Some(r#"
  _____
 /     \
|  ( )  |
 \_____/
    |
   _|_
  |___|
    | "#),

        "book" => Some(r#"
 ________
|        |
|  READ  |
|        |
|________|
 \______/ "#),

        "bomb" => Some(r#"
  @
 /
/ \
|   |
| * |
 \_/ "#),

        "lightning" | "bolt" => Some(r#"
    /|
   / |
  /  |
 /____|
    /
   /
  / "#),

        "alien" => Some(r#"
  ___
 /   \
| ^ ^ |
|  o  |
| --- |
/_____\ "#),

        "fire" => Some(r#"
  )
 ) \
/ ) (
\(_)/
 \_/ "#),

        "planet" | "earth" => Some(r#"
    ___
  /  -  \
 | /---\ |
 | \   / |
  \  -  /
    --- "#),

        "anchor" => Some(r#"
    _
   (_)
    |
  __|__
 /     \
|       |
|  ___  |
 \_____/ "#),

        "diamond" => Some(r#"
    /\
   /  \
  / <> \
  \    /
   \  /
    \/ "#),

        "cactus" => Some(r#"
  _|_
 |   |
 | o-|--
 |   |
-|---| |
 |   | |
 |___| "#),

        "penguin" => Some(r#"
   _~_
  (o o)
 /| ^ |\
  | # |
  |___|
  /   \ "#),

        _ => None,
    }
}

pub fn list_all() -> Vec<&'static str> {
    vec![
        "cat", "dog", "skull", "sun", "moon", "house", "tree", "sword",
        "fish", "bird", "car", "rocket", "heart", "star", "coffee", "pizza",
        "crown", "ghost", "robot", "snake", "dragon", "wave", "mountain",
        "flower", "clock", "key", "book", "bomb", "lightning", "alien",
        "fire", "planet", "anchor", "diamond", "cactus", "penguin",
    ]
}
