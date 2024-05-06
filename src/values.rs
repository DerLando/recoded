use std::any::{Any, TypeId};

use crate::pins::{IPin, InputPin, OPin, OutputPin};

/// Using a value enum is potentially quite wasteful,
/// as all variants will be 64 bits and we will have to pattern match
/// on the values inside of the iterator. This is quite a lot of
/// overhead, which I'm not sure I love...
/// I would love an api like
/// ```
/// let output = node.values_out(out_id);
/// other_node.values_in(in_id);
/// ```
/// where both `values_out` and `values_in` are generic over some
/// type `T`. There is no way to name that type for unknown nodes
/// though, so I think going for a value enum (which we can name)
/// is the correct approach for now. So the signature becomes:
/// ```
/// fn values_out(&self) -> Values {todo!()}
/// ```
///
pub enum Values {
    /// This is a u32, since u32 and f64 can be converted into eachother
    Int(Vec<u32>),
    Float(Vec<f64>),
    String(Vec<String>),
    Bool(Vec<bool>),
    Point(Vec<piet::kurbo::Point>),
    Circle(Vec<piet::kurbo::Circle>),
    Shape(Vec<crate::shapes::Shapes>),
    Custom(Vec<Box<dyn Any>>),
    // TODO: How can we allow custom types here? Box<dyn Any>?
    // Custom(Vec<T>),
    // CustomRef(Vec<Box<T>>),
}

fn pipe_values_into<P, C>(producer: &OutputPin<P>, consumer: &mut InputPin<C>)
where
    P: Clone,
    C: From<P> + PartialEq,
{
    consumer.values_in(producer.values_out().into_iter().cloned());
}
