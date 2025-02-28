// Verifies that type metadata identifiers for trait objects are emitted correctly.
//
// needs-sanitizer-cfi
// compile-flags: -Clto -Cno-prepopulate-passes -Copt-level=0 -Ctarget-feature=-crt-static -Zsanitizer=cfi

#![crate_type="lib"]

pub trait Trait1 {
    fn foo(&self);
}

#[derive(Clone, Copy)]
pub struct Type1;

impl Trait1 for Type1 {
    fn foo(&self) {
    }
}

pub trait Trait2<T> {
    fn bar(&self);
}

pub struct Type2;

impl Trait2<i32> for Type2 {
    fn bar(&self) {
    }
}

pub trait Trait3<T> {
    fn baz(&self, _: &T);
}

pub struct Type3;

impl<T, U> Trait3<U> for T {
    fn baz(&self, _: &U) {
    }
}

pub fn foo1(a: &dyn Trait1) {
    a.foo();
    // CHECK-LABEL: define{{.*}}4foo1{{.*}}!type !{{[0-9]+}}
    // CHECK:       call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%[0-9]}}, metadata !"[[TYPE1:[[:print:]]+]]")
}

pub fn bar1() {
    let a = Type1;
    let b = &a as &dyn Trait1;
    b.foo();
    // CHECK-LABEL: define{{.*}}4bar1{{.*}}!type !{{[0-9]+}}
    // CHECK:       call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%[0-9]}}, metadata !"[[TYPE2:[[:print:]]+]]")
}

pub fn foo2<T>(a: &dyn Trait2<T>) {
    a.bar();
    // CHECK-LABEL: define{{.*}}4foo2{{.*}}!type !{{[0-9]+}}
    // CHECK:       call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%[0-9]}}, metadata !"[[TYPE2:[[:print:]]+]]")
}

pub fn bar2() {
    let a = Type2;
    foo2(&a);
    let b = &a as &dyn Trait2<i32>;
    b.bar();
    // CHECK-LABEL: define{{.*}}4bar2{{.*}}!type !{{[0-9]+}}
    // CHECK:       call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%[0-9]}}, metadata !"[[TYPE2:[[:print:]]+]]")
}

pub fn foo3(a: &dyn Trait3<Type3>) {
    let b = Type3;
    a.baz(&b);
    // CHECK-LABEL: define{{.*}}4foo3{{.*}}!type !{{[0-9]+}}
    // CHECK:       call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%[0-9]}}, metadata !"[[TYPE3:[[:print:]]+]]")
}

pub fn bar3() {
    let a = Type3;
    foo3(&a);
    let b = &a as &dyn Trait3<Type3>;
    b.baz(&a);
    // CHECK-LABEL: define{{.*}}4bar3{{.*}}!type !{{[0-9]+}}
    // CHECK:       call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%[0-9]}}, metadata !"[[TYPE3:[[:print:]]+]]")
}

// CHECK: !{{[0-9]+}} = !{i64 0, !"[[TYPE1]]"}
// CHECK: !{{[0-9]+}} = !{i64 0, !"[[TYPE2]]"}
// CHECK: !{{[0-9]+}} = !{i64 0, !"[[TYPE3]]"}
