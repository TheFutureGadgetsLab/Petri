using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class FoodSpawner : MonoBehaviour
{
    GameObject FoodPrefab;

    List<GameObject> CellPrefabs;

    FoodParams foodConfig;
    Bounds bounds;

    private float curTime;

    void Start()
    {
        FoodPrefab = Resources.Load<GameObject>("Food");

        foodConfig = GameObject.Find("Settings").GetComponent<Settings>().foodParams;
        bounds = GameObject.Find("Bounds").GetComponent<Bounds>();

        curTime = foodConfig.spawnInterval;
    }

    // Update is called once per frame
    void FixedUpdate()
    {
        spawnFood();
    }

    void spawnFood()
    {
        curTime -= Time.fixedDeltaTime;
        if (curTime >= 0) {
            return;
        }
        curTime = foodConfig.spawnInterval;

        var nFood  = (int)Random.Range(foodConfig.spawnNum.min, foodConfig.spawnNum.max);
        for (int i = 0; i < nFood; i++)
        {
            var food = GameObject.Instantiate(
                FoodPrefab, 
                bounds.GetRandomPos(),
                Quaternion.identity
            );
            food.transform.localScale = foodConfig.scale;
        }

    }
}
