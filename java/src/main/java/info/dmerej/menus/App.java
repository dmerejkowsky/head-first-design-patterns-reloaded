package info.dmerej.menus;

import java.util.List;

public class App {
    public static void main(String[] args) {
        var blt = new MenuItem("BLT", 3, false);
        var hotDog = new MenuItem("Hot dog", 2, false);
        var pancake = new MenuItem("Pancake", 10, true);
        var waffles = new MenuItem("Waffles", 7, true);
        var pancakeHouseMenu = new Menu("Pancake House", List.of(pancake, waffles));
        var dinerMenu = new Menu("Diner Menu", List.of(blt, hotDog, pancakeHouseMenu));
        var allMenus = new Menu("All menus", List.of(dinerMenu));
        var waitress = new Waitress(allMenus);
        waitress.printMenus();
    }
}