using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class EnergySpawner : MonoBehaviour
{
    GameObject EnergyPrefab;

    List<GameObject> CellPrefabs;

    Bounds bounds;

    private float curTime;

    void Start()
    {
        EnergyPrefab = Resources.Load<GameObject>("Energy");

        bounds = GameObject.Find("Bounds").GetComponent<Bounds>();

        curTime = Settings.inst.energy.spawnInterval;
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
        curTime = Settings.inst.energy.spawnInterval;

        var nEnergy  = (int)Random.Range(Settings.inst.energy.spawnNum.min, Settings.inst.energy.spawnNum.max);
        for (int i = 0; i < nEnergy; i++)
        {
            var energy = GameObject.Instantiate(
                EnergyPrefab, 
                bounds.GetRandomPos(),
                Quaternion.identity
            );
            energy.transform.localScale = Settings.inst.energy.scale;
        }

    }
}
