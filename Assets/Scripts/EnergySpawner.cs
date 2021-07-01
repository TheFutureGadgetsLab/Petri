using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class EnergySpawner : MonoBehaviour
{
    GameObject EnergyPrefab;

    List<GameObject> CellPrefabs;

    EnergyParams energyConfig;
    Bounds bounds;

    private float curTime;

    void Start()
    {
        EnergyPrefab = Resources.Load<GameObject>("Energy");

        energyConfig = GameObject.Find("Settings").GetComponent<Settings>().energyParams;
        bounds = GameObject.Find("Bounds").GetComponent<Bounds>();

        curTime = energyConfig.spawnInterval;
    }

    // Update is called once per frame
    void FixedUpdate()
    {
        spawnEnergy();
    }

    void spawnEnergy()
    {
        curTime -= Time.fixedDeltaTime;
        if (curTime >= 0) {
            return;
        }
        curTime = energyConfig.spawnInterval;

        var nEnergy  = (int)Random.Range(energyConfig.spawnNum.min, energyConfig.spawnNum.max);
        for (int i = 0; i < nEnergy; i++)
        {
            var energy = GameObject.Instantiate(
                EnergyPrefab, 
                bounds.GetRandomPos(),
                Quaternion.identity
            );
            energy.transform.localScale = energyConfig.scale;
        }

    }
}
