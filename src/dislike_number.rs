use anyhow::Result;

fn solve(dislike_number:u32, rooms:Vec<u32>) -> Option<Vec<u32>>{
    let mut results = Vec::new();
    for room in rooms {
        if ! is_dislike_room(dislike_number, room) {
            results.push(room);
        }
    }
    if results.len() == 0 {
        return None;
    }
    Some(results)
}

fn is_dislike_room(dislike_number:u32, room:u32) -> bool {
    let room_str = room.to_string();
    for ch in room_str.chars() {
        let d = ch.to_digit(10).unwrap();
        if d == dislike_number {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        let rooms = vec![101, 204, 301, 401, 501];
        let want = vec![101, 301, 501];
        let got = super::solve(4, rooms).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test1() {
        let rooms = vec![409, 509, 109];
        let want = None;
        let got = super::solve(9, rooms);
        assert_eq!(want, got);
    }

    #[test]
    fn test2() {
        let rooms = vec![101, 102, 205, 224, 231, 314];
        let want = vec![205, 224];
        let got = super::solve(1, rooms).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_is_dislike_room() {
        assert_eq!(true, super::is_dislike_room(4, 104));
        assert_eq!(false, super::is_dislike_room(4, 105));
    }
}
