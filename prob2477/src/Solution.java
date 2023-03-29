import java.util.ArrayList;
import java.util.HashSet;

class Solution {
    public long minimumFuelCost(int[][] roads, int seats) {


        ArrayList<ArrayList<Integer>> childrenGraph = new ArrayList<>();
        for(int i = 0, n = roads.length; i < n; i++)
            childrenGraph.add(new ArrayList<>());

        for(var road: roads) {
            childrenGraph.get(road[0]).add(road[1]);
            childrenGraph.get(road[1]).add(road[0]);
        }

        int[] d = new int[1];
        for(var child: childrenGraph.get(0))
            dfs(childrenGraph, seats, d, 0, child);

        return d[0];
    }

    public int dfs(ArrayList<ArrayList<Integer>> childrenGraph, int carSeats, int[] d, int inc, int i) {
        int passengers = 1;
        for(var child: childrenGraph.get(i))
            if (child != inc)
                passengers += dfs(childrenGraph, carSeats, d, i, child);

        // number of cars needed from here to top
        d[0] += passengers / carSeats;
        d[0] += (passengers % carSeats == 0) ? 0 : 1;
        return passengers;
    }
}