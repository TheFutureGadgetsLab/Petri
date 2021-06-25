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
        
        CellPrefabs = new List<GameObject>(){
            Resources.Load<GameObject>("Cell"),
            Resources.Load<GameObject>("Propulsion")
        };

        config = GameObject.Find("Settings").GetComponent<Settings>().foodParams;
        bounds = GameObject.Find("Bounds").GetComponent<Bounds>();

        curTime = config.spawnInterval;
    }

    // Update is called once per frame
    void FixedUpdate()
    {
        spawnFood();
        transformFood();
    }

    void transformFood()
    {
        // Copy list so we can remove during iter
        // Can probably use reverse iterator to remove copy
        var copy = Food.instances.ToList();
        foreach (var obj in copy) {
            var inst = obj.GetComponent<Food>();
            if (inst.food < config.toCellThresh) {
                continue;
            }

            var newCell = GameObject.Instantiate(
                CellPrefabs[(int)Random.Range(0, CellPrefabs.Count)], 
                obj.transform.position,
                Quaternion.identity
            );
            newCell.transform.localScale = config.scale;
            newCell.GetComponent<Cell>().food = inst.food;

            GameObject.Destroy(obj);
        }
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
