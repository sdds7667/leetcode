import java.util.Arrays;

public class QuickSort {
    public static void main(String[] args) {
        int[] arr = new int[]{6, 4, 9, 4, 3, 1, 5, 2, 3, 3, 2, 2, 2, 6, 8};
        quicksort(arr, 0, arr.length - 1);
        System.out.println(Arrays.toString(arr));
    }


    public static void quicksort(int[] arr, int start, int end) {
        if (start >= end || start < 0) return;
        var p = partition(arr, start, end);
        quicksort(arr, start, p);
        quicksort(arr, p + 1, end);
    }

    public static int partition(int[] arr, int start, int end) {
        int pivot = arr[(start + end) / 2];
        int i = start - 1;
        int j = end + 1;
        while (true) {
            do i++;
            while (arr[i] < pivot);
            do j--;
            while(arr[j] > pivot);
            if (i >= j) return j;
            int t = arr[i];
            arr[i] = arr[j];
            arr[j] = t;
        }
    }
}
