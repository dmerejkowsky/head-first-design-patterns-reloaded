package info.dmerej.pizza;

import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

public class PizzaTest {
    @Test
    void new_york_uses_sliced_pepperoni_and_fresh_clams() {
        var factory = new NewYorkIngredients();
        var pepperoni = factory.createPepperoni();
        assertThat(pepperoni).isInstanceOf(SlicedPepperoni.class);
        var clams = factory.createClams();
        assertThat(clams).isInstanceOf(FreshClams.class);
    }

    @Test
    void chicago_uses_sliced_pepperoni_and_frozen_clams() {
        var factory = new ChicagoIngredients();
        var pepperoni = factory.createPepperoni();
        assertThat(pepperoni).isInstanceOf(SlicedPepperoni.class);
        var clams = factory.createClams();
        assertThat(clams).isInstanceOf(FrozenClams.class);
    }
}
