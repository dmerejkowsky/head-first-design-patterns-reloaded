package info.dmerej.barrista;

public abstract class HotBeverage {
    public abstract void prepareRecipe();

    void boilWater() {
        System.out.println("Boiling water");
    }

    void pourInCup() {
        System.out.println("Pouring in cup");
    }

}
