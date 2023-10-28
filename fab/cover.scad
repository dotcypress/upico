$fn=128;
epsilon=0.01;

module cover() {
  difference() {
    union() {
      cube(size=[58, 9.5, 1], center=true);
      translate([0, 0, 1])
        cube(size=[55, 9.5, 1.8], center=true);
    }

    translate([5.3+epsilon, 2.5+epsilon, 0]) 
      cube(size=[44, 5, 20], center=true);

    translate([-24.5, 1.8, 0]) hull() {
      cylinder(r=1.8, h=20, center=true);
      translate([10, 0, 0])
        cylinder(r=1.8, h=20, center=true);
    }
  }
}

module print() {
  translate([0, 4, 0.5]) 
    cube(size=[1, 88, 1], center=true);
  for (i=[-3:4]) {
    translate([0, i*12, 0.6]) cover();
  }
}

// print();
cover();