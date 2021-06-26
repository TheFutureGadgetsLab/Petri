using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class FoodSpawner : MonoBehaviour
{
    GameObject FoodPrefab;

    List<GameObject> CellPrefabs;

    FoodParams config;
    Bounds bounds;

    private float curTime;

    void Start()
    {
        FoodPrefab = Resources.Load<GameObject>("Food");

        config = GameObject.Find("Settings").GetComponent<Settings>().foodParams;
        bounds = GameObject.Find("Bounds").GetComponent<Bounds>();

        curTime = config.spawnInterval;
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
        curTime = config.spawnInterval;

        var nFood  = (int)Random.Range(config.spawnNum.min, config.spawnNum.max);
        for (int i = 0; i < nFood; i++)
        {
            var food = GameObject.Instantiate(
                FoodPrefab, 
                bounds.GetRandomPos(),
                Quaternion.identity
            );
            food.transform.localScale = config.scale;
        }

    }
}
