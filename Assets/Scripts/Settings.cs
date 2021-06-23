using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;

[Serializable]
public class Settings : MonoBehaviour
{
    public FoodParams foodParams;
    public CellParams cellParams;
    public PropulsionParams propulsionParams;
    public BoundsParams boundsParams;
}

[Serializable]
public struct BoundsParams
{
    public float size;
}

[Serializable]
public struct PropulsionParams
{
    public MinMaxF force;
    public MinMaxF torque;
    public float   cost;
}


[Serializable]
public struct FoodParams
{
    public MinMaxI spawnNum;
    public MinMaxF value;
    public float spawnInterval; // Spawn interval in seconds
    public Vector2 scale;
    public MinMaxF velocity;
}

[Serializable]
public struct CellParams
{
    public int spawnNum;
    public Vector2 scale;
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