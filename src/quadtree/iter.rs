use macroquad::math::{Circle, Vec2};

use super::quadtree::{Positionable, QuadTree};

pub struct QuadTreeIter<T> {
    stack: Vec<T>,
}

impl<T> Iterator for QuadTreeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<'a, T: Positionable> IntoIterator for &'a mut QuadTree<T> {
    type Item = T;
    type IntoIter = QuadTreeIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack: Vec<T> = Vec::new();
        let mut node_stack = vec![&mut self.root];

        while let Some(node) = node_stack.pop() {
            while let Some(pt) = node.points.pop() {
                stack.push(pt);
            }

            for child in &mut node.children {
                node_stack.push(child);
            }
        }

        QuadTreeIter { stack }
    }
}

impl<T> QuadTree<T>
where
    T: Positionable,
{
    pub fn iter(&self) -> QuadTreeIter<&T> {
        let mut stack: Vec<&T> = Vec::new();
        let mut node_stack = vec![&self.root];

        while let Some(node) = node_stack.pop() {
            for pt in &node.points {
                stack.push(pt);
            }

            for child in &node.children {
                node_stack.push(child);
            }
        }

        QuadTreeIter { stack }
    }

    pub fn iter_mut(&mut self) -> QuadTreeIter<&mut T> {
        let mut stack: Vec<&mut T> = Vec::new();
        let mut node_stack = vec![&mut self.root];

        while let Some(node) = node_stack.pop() {
            for pt in &mut node.points {
                stack.push(pt);
            }

            for child in &mut node.children {
                node_stack.push(child);
            }
        }

        QuadTreeIter { stack }
    }

    pub fn query(&self, position: Vec2, range: f32) -> Vec<&T> {
        let circle = Circle::new(position.x, position.y, range);

        let mut stack: Vec<&T> = Vec::new();
        let mut node_stack = vec![&self.root];

        while let Some(node) = node_stack.pop() {
            if node.points.len() > 0 && circle.overlaps_rect(&node.bound) {
                for point in &node.points {
                    stack.push(point);
                }
            }

            for child in &node.children {
                node_stack.push(child);
            }
        }

        stack
    }
}
