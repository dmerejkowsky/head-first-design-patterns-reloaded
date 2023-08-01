package info.dmerej.menus;

import java.util.List;

public class Menu implements MenuComponent {
    private final List<MenuComponent> components;
    private final String name;

    public Menu(String name, List<MenuComponent> components) {
        this.components = components;
        this.name = name;
    }

    @Override
    public String getName() {
        return name;
    }

    @Override
    public int getPrice() {
        return components.stream().mapToInt(c -> c.getPrice()).sum();
    }

    @Override
    public boolean isVegetarian() {
        return components.stream().allMatch(c -> c.isVegetarian());
    }

    @Override
    public void add(MenuComponent component) {
        components.add(component);
    }

    @Override
    public MenuComponent getChild(int index) {
        return components.get(index);
    }

    @Override
    public void print() {
        System.out.println(this.name);
        for (var component : components) {
            component.print();
        }
    }
}
