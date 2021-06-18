using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class FoodSpawner : MonoBehaviour
{
    GameObject Food;
    public Vector2 minMaxSpawn = new Vector2(1, 2);
    public Vector2 FoodScale = new Vector2(1, 1);
    public float spawnTime = 2; // Seconds
    private float curTime;
    void Start()
    {
        curTime = spawnTime;
        Food = Resources.Load<GameObject>("Food");
    }

    // Update is called once per frame
    void FixedUpdate()
    {
        curTime -= Time.fixedDeltaTime;
        if (curTime >= 0) {
            return;
        }
        curTime = spawnTime;

        var nFood  = (int)Random.Range(minMaxSpawn.x, minMaxSpawn.y);
        var bounds = GameObject.Find("Bounds").GetComponent<Bounds>();
        for (int i = 0; i < 1; i++)
        {
            var food = GameObject.Instantiate(
                Food, 
                bounds.GetRandomPos(),
                Quaternion.identity
            );
            food.transform.localScale = FoodScale;
        }
    }
}
