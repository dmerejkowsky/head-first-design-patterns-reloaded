package info.dmerej.barrista;

public class Coffee {
    public void prepareRecipe() {
        boilWater();
        brewCoffeeGrains();
        pourInCup();
        addSugarAndMilk();
    }

    private void boilWater() {
        System.out.println("Boiling water");
    }

    private void brewCoffeeGrains() {
        System.out.println("Brewing coffee grains");
    }

    private void pourInCup() {
        System.out.println("Pouring in cup");
    }

    private void addSugarAndMilk() {
        System.out.println("Adding sugar and milk");
    }

}
