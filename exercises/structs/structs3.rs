// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            panic!("Can not ship a weightless package.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> bool {
        return self.sender_country != self.recipient_country;
    }

    fn get_fees(&self, cents_per_gram: i32) -> i32{
        self.weight_in_grams * cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}

// 结构体定义：

// #[derive(Debug)]：这个宏允许结构体实例使用调试格式进行打印，方便调试。
// struct Package：定义了一个名为 Package 的结构体，包含三个字段：
// sender_country: String：寄件国家。
// recipient_country: String：收件国家。
// weight_in_grams: i32：包裹重量，以克为单位。
// 实现块 impl Package：

// fn new(...) -> Package：关联函数，用于创建新的 Package 实例。如果输入的重量小于等于0，则程序会崩溃并显示错误信息。
// fn is_international(&self) -> bool：方法判断包裹是否是国际包裹。如果寄件国和收件国不同，则视为国际包裹，返回 true。
// fn get_fees(&self, cents_per_gram: i32) -> i32：方法计算运费。运费等于包裹重量乘以每克的费用（以美分为单位）。
// 测试模块 #[cfg(test)] mod tests：

// 使用 #[cfg(test)] 标记这个模块只在测试时编译。
// use super::*;：引入外部模块的所有内容，以便进行测试。
// 定义了几个测试用例：
// fail_creating_weightless_package：测试尝试创建一个重量为负数的包裹，会导致程序崩溃。
// create_international_package：测试创建一个国际包裹的功能，判断是否返回 true。
// create_local_package：测试创建一个国内包裹的功能，判断是否返回 false。
// calculate_transport_fees：测试运费计算功能，验证给定的重量和费用是否正确计算出预期的运费。