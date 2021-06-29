using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Weapon : Cell
{
    WeaponParams config;

    Vector2 force;
    float torque;

    new private void Awake() {
        base.Awake();
        config = GameObject.Find("Settings").GetComponent<Settings>().weaponParams;
    }

    new private void FixedUpdate()
    {
        base.FixedUpdate();

    }
}

