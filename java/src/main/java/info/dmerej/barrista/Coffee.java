package info.dmerej.barrista;

public class Coffee extends HotBeverage {
    public void prepareRecipe() {
        boilWater();
        brewCoffeeGrains();
        pourInCup();
        addSugarAndMilk();
    }


    private void brewCoffeeGrains() {
        System.out.println("Brewing coffee grains");
    }


    private void addSugarAndMilk() {
        System.out.println("Adding sugar and milk");
    }

}
