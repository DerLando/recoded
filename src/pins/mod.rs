#[derive(Default)]
pub struct InputPinId(usize);
#[derive(Default)]
pub struct OutputPinId(usize);
#[derive(Default)]
pub struct InputPin<T> {
    id: InputPinId,
    data: PinData<T>,
}

#[derive(Default)]
pub struct OutputPin<T> {
    id: OutputPinId,
    data: PinData<T>,
}

#[derive(Default)]
struct PinData<T> {
    data: Vec<T>,
    is_dirty: bool,
}

trait IPin<T> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Default)]
    struct TestNode {
        count: InputPin<usize>,
        number: InputPin<f64>,
        series_out: OutputPin<f64>,
    }

    impl TestNode {
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

    #[test]
    fn can_do_ops_with_testnode() {
        let mut node = TestNode::default();
        node.count.value_in(10u8);
        node.number.value_in(42.0f32);
        let actual = node.get_series_out();
        assert!(actual.iter().all(|n| *n == 42.0f64));
    }
}
