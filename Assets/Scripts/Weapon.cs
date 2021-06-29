using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class Weapon : Cell
{
    WeaponParams weaponConfig;

    Vector2 force;
    float torque;

    new private void Awake() {
        base.Awake();
        weaponConfig = GameObject.Find("Settings").GetComponent<Settings>().weaponParams;
    }

    new private void FixedUpdate()
    {
        base.FixedUpdate();
        Collider2D[] near = Physics2D.OverlapCircleAll(transform.position, 2.0f);
    }
}

