using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class FoodSpawner : MonoBehaviour
{
    GameObject Food;
    FoodParams config;
    Bounds bounds;

    private float curTime;

    void Start()
    {
        Food = Resources.Load<GameObject>("Food");
        config = GameObject.Find("Settings").GetComponent<Settings>().foodParams;
        bounds = GameObject.Find("Bounds").GetComponent<Bounds>();

        curTime = config.spawnInterval;
    }

    // Update is called once per frame
    void FixedUpdate()
    {
        curTime -= Time.fixedDeltaTime;
        if (curTime >= 0) {
            return;
        }
        curTime = config.spawnInterval;

        var nFood  = (int)Random.Range(config.spawnNum.min, config.spawnNum.max);
        for (int i = 0; i < 1; i++)
        {
            var food = GameObject.Instantiate(
                Food, 
                bounds.GetRandomPos(),
                Quaternion.identity
            );
            food.transform.localScale = config.scale;
        }
    }
}
