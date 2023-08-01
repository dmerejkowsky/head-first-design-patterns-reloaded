package info.dmerej.menus;

public interface MenuComponent {
    String getName();

    int getPrice();

    boolean isVegetarian();

    void add(MenuComponent component);

    MenuComponent getChild(int index);

    void print();
}
