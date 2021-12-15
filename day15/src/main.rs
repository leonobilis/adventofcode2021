use std::fs;
use petgraph::graphmap::DiGraphMap;
use petgraph::algo::dijkstra;


fn parse_input(inp: String) -> Vec<Vec<u32>> {
    inp.lines()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
        )
        .collect()
}

fn get_graph(grid: &Vec<Vec<u32>>) -> DiGraphMap::<(usize, usize), &u32> {
    let mut g = DiGraphMap::<(usize, usize), &u32>::new();

    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| line
            .iter()
            .enumerate()
            .map(move |(x, val)| ((x, y), val))
        )
        .for_each(|((x, y), val)| {
            if x > 0 {
                g.add_edge((x-1, y), (x, y), val);
            }
            if x < grid.len() - 1 {
                g.add_edge((x+1, y), (x, y), val);
            }
            if y > 0 {
                g.add_edge((x, y-1), (x, y), val);
            }
            if y < grid.len() - 1 {
                g.add_edge((x, y+1), (x, y), val);
            }
    });
    g
}

fn p1(grid: &Vec<Vec<u32>>) -> u32 {
    let g = get_graph(grid);

    let start = (0, 0);
    let end = (grid.len() -1, grid.len() -1);

    let d = dijkstra(&g, start, Some(end), |edge| **edge.2);
    *d.get(&end).unwrap()
}

fn p2(grid: &Vec<Vec<u32>>) -> u32 {
    let grid5: Vec<Vec<u32>> = (0..grid.len()*5)
        .map(|y| (0..grid.len()*5)
            .map(|x| *grid.get(y%grid.len()).unwrap().get(x%grid.len()).unwrap() + (x/grid.len() + y/grid.len()) as u32)
            .map(|val| if val < 10 { val } else { val%9 } )
            .collect()
        )
        .collect();

    let g = get_graph(&grid5);

    let start = (0, 0);
    let end = (grid5.len() -1, grid5.len() -1);

    let d = dijkstra(&g, start, Some(end), |edge| **edge.2);
    *d.get(&end).unwrap()
}

fn main() {

    let inp = fs::read_to_string("input.txt").unwrap();
    let grid = parse_input(inp);

    println!("Part 1: {}", p1(&grid));
    println!("Part 2: {}", p2(&grid));

}