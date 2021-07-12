struct Monster {
    attack: i32,
    defence: i32,
    speed: i32,
}

impl Monster {
    fn new(attack:i32, defence:i32, speed:i32) -> Monster {
        Monster{attack,defence, speed}
    }
}

struct EvoMonster {
    name: String,
    min_attack: i32,
    max_attack: i32,
    min_defenece: i32,
    max_defence: i32,
    min_speed: i32,
    max_speed: i32,
}

impl EvoMonster {
    fn new(name:&str, min_attack:i32, max_attack:i32, min_defenece:i32, max_defence:i32, min_speed:i32, max_speed:i32) -> EvoMonster {
        EvoMonster{name:name.to_string(), min_attack, max_attack, min_defenece, max_defence, min_speed, max_speed}
    }

    fn evolvable(&self, mst:&Monster) -> bool {
        self.min_attack <= mst.attack && mst.attack <= self.max_attack
         && self.min_defenece <= mst.defence && mst.defence <= self.max_defence
         && self.min_speed <= mst.speed && mst.speed <= self.max_speed
    }
}

fn solve(monster:Monster, evo_monsters:Vec<EvoMonster>) -> Vec<String> {
    let mut results = Vec::new();
    for evo_monster in evo_monsters {
        if evo_monster.evolvable(&monster){
            results.push(evo_monster.name);
        }
    }
    if results.is_empty() {
        results.push("no evolution".to_string());
    }
    results
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_evolvable(){
        let mst = super::Monster::new(100, 150, 200);
        let bird = super::EvoMonster::new("paizabird", 100,200,130,180,80,120);
        let wolf = super::EvoMonster::new("paizawolf", 180,220,100,120,90,140);
        let sheep = super::EvoMonster::new("paizasheep", 80,110,150,220,170,250);
        assert_eq!(bird.evolvable(&mst), false);
        assert_eq!(wolf.evolvable(&mst), false);
        assert_eq!(sheep.evolvable(&mst), true);
    }

    #[test]
    fn test_solve0(){
        let mst = super::Monster::new(100, 150, 200);
        let mut evos = Vec::new();
        evos.push(super::EvoMonster::new("paizabird", 100,200,130,180,80,120));
        evos.push(super::EvoMonster::new("paizawolf", 180,220,100,120,90,140));
        evos.push(super::EvoMonster::new("paizasheep", 80,110,150,220,170,250));
        let want = vec!["paizasheep"];
        let got = super::solve(mst, evos);
        assert_eq!(want,got);
    }

    #[test]
    fn test_solve1(){
        let mst = super::Monster::new(937, 405, 932);
        let evos = vec![super::EvoMonster::new("qbng", 321, 805, 256, 773, 210, 521)];
        let got = super::solve(mst, evos);
        let want = vec!["no evolution"];
        assert_eq!(want, got);
    }
}