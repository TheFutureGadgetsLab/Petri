using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;

[Serializable]
public class Settings : MonoBehaviour
{
    public EnergyParams energy;
    public CellParams cell;
    public PropulsionParams propulsion;
    public BoundaryParams boundary;
    public WeaponParams weapon;

    public static Settings inst;

    private void OnEnable() {
        inst = this;
        Cell.prefabs = new List<(GameObject prefab, float chance)>{
            ((Resources.Load<GameObject>("Cell"), 0.6f)),
            ((Resources.Load<GameObject>("Propulsion"), 0.3f)),
            ((Resources.Load<GameObject>("Weapon"), 0.1f))
        };
    }
}

[Serializable]
public struct BoundaryParams
{
    public float width;
    public float height;
    public float thickness;
}

[Serializable]
public struct WeaponParams
{
    public float attackCost;
    public float attackRadius;
    public float drainRate;
}

[Serializable]
public struct PropulsionParams
{
    public float   force;
    public MinMaxF torque;
    public float   cost;
    public float   speedLimit;
}


[Serializable]
public struct EnergyParams
{
    public MinMaxI spawnNum;
    public MinMaxF value;
    public float spawnInterval; // Spawn interval in seconds
    public Vector2 scale;
    public MinMaxF velocity;
    public float toCellThresh;
}

[Serializable]
public struct CellParams
{
    public Vector2 scale;
    public float shareRate;
    public float bondForce;
    public int maxBonds;
    public float minEnergy;
}

[Serializable]
public struct MinMaxI
{
    public int min;
    public int max;

    public int sample()
    {
        return UnityEngine.Random.Range(min, max);
    }
}


[Serializable]
public struct MinMaxF
{
    public float min;
    public float max;

    public float sample()
    {
        return UnityEngine.Random.Range(min, max);
    }
}
