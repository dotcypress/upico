$fn=128;
epsilon=0.01;

module cover() {
  difference() {
    union() {
      cube(size=[58, 10, 1.4], center=true);
      translate([0, -2.5, 0.95])
        cube(size=[55, 5, 2.5], center=true);
      translate([-22, 0, 0.95]) 
        cube(size=[12, 10, 2.5], center=true);
    }

    translate([5.5, 2.5+epsilon, 0]) 
      cube(size=[44, 5, 5], center=true);

    translate([-24.5, 1.8, 0]) hull() {
      cylinder(r=1.8, h=10, center=true);
      translate([26.5, 0, 0])
        cylinder(r=1.8, h=10, center=true);
    }
  }
}

cover();
