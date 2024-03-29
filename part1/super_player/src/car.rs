// 特征继承
// 和面向对象语言相比，特征及其实现类似接口和实现这些接口的类
pub trait Vehicle {
    fn get_price(&self) -> u64;
}

// 对于Car trait的具体实现者必须要实现Vehicle上的get_price方法
// ===rust特征与其他语言的接口存在很大差异===
// 1.在rust中类型本身并没有任何继承。因此采用的是类型组合而不是对象继承，它依赖
// 于特征继承来为代码中的任何实际的实体建模。
// 2.可以在任何地方编写特征实现的代码块，而且无须访问实际类型
// 3.可以基于内置的基元类型到泛型之间的任何类型实现自定义特征
// 4.在函数中不能隐式地将返回类型作为特征，就像在 Go/Java 中可以将接口作为返回类
// 型，你必须返回一个被称为特征对象的东西，并且这种声明是显式的。
pub trait Car: Vehicle {
    fn model(&self) -> String;
}
