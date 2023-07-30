package info.dmerej.menus;

import java.util.ArrayList;
import java.util.stream.Stream;

public class PancakeHouseMenu implements Menu {
    private final ArrayList<MenuItem> items;

    PancakeHouseMenu() {
        items = new ArrayList<>();
        addItem("Regular Pancake", 10);
        addItem("Waffles", 7);
    }

    private void addItem(String name, int price) {
        var item = new MenuItem(name, price);
        items.add(item);
    }

    public Stream<MenuItem> getItems() {
        return items.stream();
    }
}
