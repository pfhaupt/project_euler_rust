// https://projecteuler.net/problem=28

struct Agent {
    x: usize,
    y: usize
}
const SIZE: usize = 11;

fn main() {
    let mut arr = vec![vec![0; SIZE]; SIZE];
    let mut agent = Agent { x: SIZE / 2, y: SIZE / 2 };
    let mut index = 1;
    let mut step = 1;
    let mut step_ctr = 1;
    let mut dir = 3;
    loop {
        if index - 1 >= SIZE * SIZE {
            break;
        }
        arr[agent.x][agent.y] = index;
        agent = next_move(agent, dir);
        if index % step == 0 {
            dir = (dir + 1) % 4;
            step_ctr += 1;
            if step_ctr % 2 == 0 {
                step += 1;
            }
        }
        index += 1;
    }
    let mut sum = 0;
    for i in 0..SIZE {
        let diagonal1 = arr[i][i];
        let diagonal2 = arr[i][SIZE - i - 1];
        sum += diagonal1 + diagonal2;
    }
    sum -= 1; // we count the middle cell (1) twice

    println!("{sum}");
}

fn next_move(agent: Agent, dir: usize) -> Agent {
    let result;
    match dir {
        0 => result = Agent { x: agent.x + 1, y: agent.y},
        1 => result = Agent { x: agent.x, y: agent.y - 1},
        2 => result = Agent { x: agent.x - 1, y: agent.y},
        3 => result = Agent { x: agent.x, y: agent.y + 1},
        _ => panic!("Unreachable")
    }
    result
}