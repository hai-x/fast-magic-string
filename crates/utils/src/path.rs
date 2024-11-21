pub fn get_relative_path(from: &str, to: &str) -> String {
  let from_parts: Vec<&str> = from.split(&['/', '\\'][..]).collect();
  let to_parts: Vec<&str> = to.split(&['/', '\\'][..]).collect();

  let mut from_parts_clone = from_parts.clone();
  from_parts_clone.pop();

  let mut common_length = 0;
  while common_length < from_parts_clone.len() && common_length < to_parts.len() {
    if from_parts_clone[common_length] == to_parts[common_length] {
      common_length += 1;
    } else {
      break;
    }
  }

  let mut relative_path = Vec::new();

  for _ in common_length..from_parts_clone.len() {
    relative_path.push("..");
  }

  for part in &to_parts[common_length..] {
    relative_path.push(part);
  }

  relative_path.join("/")
}
