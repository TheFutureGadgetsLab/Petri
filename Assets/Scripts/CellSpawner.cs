using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CellSpawner : MonoBehaviour
{
    GameObject Jeff;
    CellParams config;

    void Start()
    {
        config = GameObject.Find("Settings").GetComponent<Settings>().cellParams;

        List<GameObject> jeffs = new List<GameObject>(){
            Resources.Load<GameObject>("Cell"),
            Resources.Load<GameObject>("Propulsion")
        };

        var bounds = GameObject.Find("Bounds").GetComponent<Bounds>();

        for (int i = 0; i < config.spawnNum; i++)
        {
            var jeff = GameObject.Instantiate(
                jeffs[i % 2], 
                bounds.GetRandomPos(),
                Quaternion.identity
            );
            jeff.transform.localScale = config.scale;
        }
    }
}
