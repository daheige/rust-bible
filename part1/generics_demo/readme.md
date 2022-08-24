# generics
泛型设计，支持函数，结构体，枚举等rust所有类型

# nm 查看泛型函数的单态化
过滤关键字，发现give_me函数，在编译后产生了2个版本
% nm target/debug/generics_demo | grep "give_me"
0000000100001550 t __ZN13generics_demo7give_me17h2374d02834384e67E
00000001000015b0 t __ZN13generics_demo7give_me17hf2610afb784dbfe7E
