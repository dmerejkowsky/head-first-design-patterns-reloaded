package info.dmerej.simuduck;

import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

public class DuckTest {
    @Test
    void mallard_duck() {
        var duck = new MallardDuck();
        assertThat(duck.display()).isEqualTo("Mallard");
        assertThat(duck.fly()).isEqualTo("I'm flying with wings");
        assertThat(duck.quack()).isEqualTo("quack");
    }

    @Test
    void redhead_duck() {
        var duck = new RedheadDuck();
        assertThat(duck.display()).isEqualTo("Red head");
        assertThat(duck.fly()).isEqualTo("I'm flying with wings");
        assertThat(duck.quack()).isEqualTo("quack");
    }

    @Test
    void rubber_duck() {
        var duck = new RubberDuck();
        assertThat(duck.display()).isEqualTo("Rubber duck");
        assertThat(duck.fly()).isEqualTo("No way");
        assertThat(duck.quack()).isEqualTo("<silence>");
    }

    @Test
    void wooden_duck_placed_on_a_rocket() {
        var duck = new WoodenDuck();
        assertThat(duck.display()).isEqualTo("Wooden duck");
        assertThat(duck.fly()).isEqualTo("No way");
        assertThat(duck.quack()).isEqualTo("<silence>");
        duck.placeOnRocket();
        assertThat(duck.fly()).isEqualTo("I'm flying on a rocket");
    }
}
