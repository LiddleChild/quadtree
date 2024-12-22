use macroquad::math::{vec2, Rect, Vec2};

pub trait Positionable {
    fn position(&self) -> Vec2;
}

#[derive(Debug)]
pub struct QuadNode<T: Positionable> {
    pub capacity: usize,
    pub bound: Rect,
    pub points: Vec<T>,
    pub children: Vec<QuadNode<T>>,
}

impl<T> QuadNode<T>
where
    T: Positionable,
{
    fn new(position: Vec2, dimension: Vec2, capacity: usize) -> Self {
        Self {
            capacity,
            bound: Rect::new(position.x, position.y, dimension.x, dimension.y),
            points: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, pt: T) {
        if self.children.len() > 0 {
            self.add_to_children(pt);
        } else {
            self.points.push(pt);

            if self.points.len() > self.capacity {
                self.sub_divide();
                while let Some(pt) = self.points.pop() {
                    self.add_to_children(pt);
                }
            }
        }
    }

    fn add_to_children(&mut self, pt: T) {
        for bound in &mut self.children {
            if bound.contains(&pt) {
                bound.push(pt);
                break;
            }
        }
    }

    pub fn contains(&self, pt: &T) -> bool {
        self.bound.contains(pt.position())
    }

    fn sub_divide(&mut self) {
        let half_width = self.bound.w / 2.0;
        let half_height = self.bound.h / 2.0;

        let north_east: QuadNode<T> = QuadNode::new(
            self.bound.point() + vec2(half_width, 0.0),
            self.bound.size() * 0.5,
            self.capacity,
        );
        let north_west: QuadNode<T> = QuadNode::new(
            self.bound.point() + vec2(0.0, 0.0),
            self.bound.size() * 0.5,
            self.capacity,
        );
        let south_west: QuadNode<T> = QuadNode::new(
            self.bound.point() + vec2(0.0, half_height),
            self.bound.size() * 0.5,
            self.capacity,
        );
        let south_east: QuadNode<T> = QuadNode::new(
            self.bound.point() + vec2(half_width, half_height),
            self.bound.size() * 0.5,
            self.capacity,
        );

        self.children = vec![north_east, north_west, south_west, south_east];
    }
}

#[derive(Debug)]
pub struct QuadTree<T: Positionable> {
    pub root: QuadNode<T>,
    count: usize,
}

impl<T: Positionable> QuadTree<T> {
    pub fn new(width: f32, height: f32, capacity: usize) -> Self {
        Self {
            root: QuadNode::new(vec2(0.0, 0.0), vec2(width, height), capacity),
            count: 0,
        }
    }

    pub fn push(&mut self, pt: T) {
        self.root.push(pt);
        self.count += 1;
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn clear(&mut self) {
        self.root = QuadNode::new(
            vec2(0.0, 0.0),
            vec2(self.root.bound.w, self.root.bound.h),
            self.root.capacity,
        );
    }

    pub fn update(&mut self) {
        let mut reassign_points: Vec<T> = Vec::new();
        let mut node_stack = vec![&mut self.root];

        while let Some(node) = node_stack.pop() {
            let mut accu = 0;
            for i in 0..node.points.len() {
                let i = i - accu;
                if !node.contains(&node.points[i]) {
                    reassign_points.push(node.points.remove(i));
                    accu += 1;
                }
            }

            for child in &mut node.children {
                node_stack.push(child);
            }
        }

        self.count -= reassign_points.len();
        for pt in reassign_points {
            self.push(pt);
        }
    }

    pub fn bounds(&self) -> Vec<&Rect> {
        let mut bound_stack = Vec::new();
        let mut node_stack = vec![&self.root];

        while let Some(node) = node_stack.pop() {
            bound_stack.push(&node.bound);

            for child in &node.children {
                node_stack.push(child);
            }
        }

        bound_stack
    }
}
