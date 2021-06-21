using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Food : MonoBehaviour
{
    public float food = 0;

    FoodParams config;

    void Awake()
    {
        config = GameObject.Find("Settings").GetComponent<Settings>().foodParams;
        food = config.value.sample();
    }
}
