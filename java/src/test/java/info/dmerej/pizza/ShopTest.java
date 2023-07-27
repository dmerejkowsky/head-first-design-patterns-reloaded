package info.dmerej.pizza;

import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class ShopTest {

    @Test
    void order_cheese_pizza_from_new_york() {
        var shop = new NewYorkShop();
        var pizza = shop.orderPizza("cheese");
        assertThat(pizza.name()).isEqualTo("cheese");
        assertThat(pizza.getCrust()).isEqualTo("thin");
    }

    @Test
    void order_pepperoni_pizza_from_chicago() {
        var shop = new ChicagoShop();
        var pizza = shop.orderPizza("pepperoni");
        assertThat(pizza.name()).isEqualTo("pepperoni");
        assertThat(pizza.getCrust()).isEqualTo("thick");
    }
}