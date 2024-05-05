#[derive(Default)]
pub struct InputPinId(pub usize);
#[derive(Default)]
pub struct OutputPinId(pub usize);
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct InputPin<T> {
    // id: InputPinId,
    data: PinData<T>,
}

impl<T> InputPin<T> {
    pub fn is_dirty(&self) -> bool {
        self.data.is_dirty
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct OutputPin<T> {
    // id: OutputPinId,
    data: PinData<T>,
}

impl<T> OutputPin<T> {
    pub fn is_dirty(&self) -> bool {
        self.data.is_dirty
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct PinData<T> {
    data: Vec<T>,
    is_dirty: bool,
}

pub trait IPin<T> {
    fn value_in(&mut self, value: T);
    fn values_in(&mut self, values: impl Iterator<Item = T>);
}

pub trait OPin<T> {
    fn value_out(&self) -> Option<&T>;
    fn values_out(&self) -> &Vec<T>;
}

impl<T, D> IPin<T> for PinData<D>
where
    D: From<T> + PartialEq,
{
    fn value_in(&mut self, value: T) {
        let value = D::from(value);
        if self.data.len() == 0 {
            self.data = vec![value];
            self.is_dirty = true;
        } else {
            self.is_dirty = value == self.data[0];
            self.data[0] = value;
        }
    }

    fn values_in(&mut self, values: impl Iterator<Item = T>) {
        self.data = values.map(|v| D::from(v)).collect::<Vec<_>>();
        self.is_dirty = true;
    }
}

impl<T, D> IPin<T> for InputPin<D>
where
    D: From<T> + PartialEq,
{
    fn value_in(&mut self, value: T) {
        self.data.value_in(value);
    }

    fn values_in(&mut self, values: impl Iterator<Item = T>) {
        self.data.values_in(values);
    }
}

impl<T, D> IPin<T> for OutputPin<D>
where
    D: From<T> + PartialEq,
{
    fn value_in(&mut self, value: T) {
        self.data.value_in(value);
    }

    fn values_in(&mut self, values: impl Iterator<Item = T>) {
        self.data.values_in(values);
    }
}

impl<T> OPin<T> for PinData<T>
where
    T: Clone,
{
    fn value_out(&self) -> Option<&T> {
        self.data.iter().next()
    }

    fn values_out(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T> OPin<T> for InputPin<T>
where
    T: Clone,
{
    fn values_out(&self) -> &Vec<T> {
        self.data.values_out()
    }

    fn value_out(&self) -> Option<&T> {
        self.data.value_out()
    }
}

impl<T> OPin<T> for OutputPin<T>
where
    T: Clone,
{
    fn values_out(&self) -> &Vec<T> {
        self.data.values_out()
    }

    fn value_out(&self) -> Option<&T> {
        self.data.value_out()
    }
}

/// TODO: Make inputs and outputs their own collections,
/// so we can impl the draw methods on them directly using new traits
/// and simplify/fast-forward implementations at the node level
/// The problem with that is, that the pins are generic over their
/// type, so we need to wrap them in a non-generic trait, so we
/// can store them together in a backing array
// pub struct Inputs<N: const usize> {
//     pins: [InputPin<?>; N]
// }

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Default)]
    struct TestSeriesNode {
        count: InputPin<usize>,
        number: InputPin<f64>,
        series_out: OutputPin<f64>,
    }

    impl TestSeriesNode {
        fn recalc_series(&mut self) {
            self.series_out.values_in(
                (0..*self.count.value_out().unwrap()).map(|_| *self.number.value_out().unwrap()),
            );
        }
        pub fn get_series_out(&mut self) -> &Vec<f64> {
            if self.count.data.is_dirty || self.number.data.is_dirty {
                self.recalc_series();
            }
            self.series_out.values_out()
        }
    }

    #[derive(Default)]
    struct TestMulNode {
        number: InputPin<f64>,
        rhs: InputPin<f64>,
        number_out: OutputPin<f64>,
    }

    impl TestMulNode {
        fn recalc(&mut self) {
            let rhs = self.rhs.value_out().map(|v| *v).unwrap_or_default().clone();
            self.number_out
                .values_in(self.number.values_out().iter().map(|v| *v * rhs))
        }
        fn get_number_out(&mut self) -> &Vec<f64> {
            if self.number.data.is_dirty || self.rhs.data.is_dirty {
                self.recalc();
            }
            self.number_out.values_out()
        }
    }

    #[test]
    fn can_do_ops_with_testnode() {
        let mut node = TestSeriesNode::default();
        node.count.value_in(10u8);
        node.number.value_in(42.0f32);
        let actual = node.get_series_out();
        assert!(actual.iter().all(|n| *n == 42.0f64));
    }

    #[test]
    fn can_stick_nodes_into_eachother() {
        let mut series_node = TestSeriesNode::default();
        series_node.count.value_in(10u8);
        series_node.number.value_in(42.0f32);
        let mut mul_node = TestMulNode::default();
        mul_node.rhs.value_in(2u8);
        // TODO: I'm not loving the .cloned() call here.
        // Would be better if the inputs could also take references
        // fine, since sometimes the data should be shared via pointers
        mul_node
            .number
            .values_in(series_node.get_series_out().iter().cloned());
        let actual = mul_node.get_number_out();
        assert!(actual.iter().all(|n| *n == 84.0f64));
    }
}
