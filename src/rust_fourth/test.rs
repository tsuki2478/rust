pub(crate) fn first() {
    enum TrafficLight {
        RED(u32),
        GREEN(u32),
        YELLOW(u32),
    }

    trait TrafficLightTime {
        fn time(&self) -> u32;
    }
    impl TrafficLightTime for TrafficLight {
        fn time(&self) -> u32 {
            match self {
                &Self::RED(v) => v,
                &Self::GREEN(v) => v,
                &Self::YELLOW(v) => v,
            }
        }
    }
    let yellow = TrafficLight::YELLOW(10);
    let green = TrafficLight::GREEN(60);
    let red = TrafficLight::RED(120);
    println!("黄:{}秒", yellow.time());
    println!("绿:{}秒", green.time());
    println!("红:{}秒", red.time());
}

pub(crate) fn second() {
    fn sum(v: &[u32]) -> Option<u32> {
        let mut num: u32 = 0;
        for &item in v.iter() {
            let value = match num.checked_add(item) {
                Some(v) => v,
                None => return None,
            };
            num = value;
        }
        Some(num)
    }

    let v = [10, 15, 20, 25];
    if let Some(value) = sum(&v) {
        println!("{}", value);
    } else {
        println!("err")
    }
}

pub(crate) fn third() {
    trait Area {
        fn area(&self) -> f64;
        fn name(&self) -> &str;
    }

    struct Triangle {
        base: f64,
        height: f64,
        name: String,
    }

    struct Circle {
        radius: f64,
        name: String,
    }

    struct Square {
        length: f64,
        name: String,
    }

    impl Area for Triangle {
        fn area(&self) -> f64 {
            (self.base * self.height) / 2.0
        }
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl Area for Circle {
        fn area(&self) -> f64 {
            self.radius * self.radius * 3.14
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    impl Area for Square {
        fn area(&self) -> f64 {
            self.length * self.length
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    let c = Circle {
        radius: 3.0,
        name: String::from("圆形"),
    };
    let t = Triangle {
        base: 4.0,
        height: 6.0,
        name: String::from("三角形"),
    };
    let s = Square {
        length: 4.0,
        name: String::from("正方形"),
    };

    count(&c);
    count(&s);
    count(&t);
    fn count<T: Area>(area: &T) {
        println!("它是:{}, 面积是:{}", area.name(), area.area())
    }

    struct Calculator<T>
    where
        T: Area,
    {
        area: T,
    }

    impl<T> Calculator<T>
    where
        T: Area,
    {
        fn instance(&self) -> &T {
            &self.area
        }

        fn print_area(&self) {
            println!(
                "特殊:{},  面基是{}",
                self.instance().name(),
                self.instance().area()
            );
        }
    }
    let calculator = Calculator { area: c };
    calculator.print_area();
}
