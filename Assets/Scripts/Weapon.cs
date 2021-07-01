using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class Weapon : Cell
{
    WeaponParams weaponConfig;

    new private void Awake() {
        base.Awake();
        weaponConfig = GameObject.Find("Settings").GetComponent<Settings>().weaponParams;
    }

    new private void FixedUpdate()
    {
        base.FixedUpdate();
        attack();
    }

    void attack()
    {
        if (energy < weaponConfig.attackCost) {
            return;
        }

        Collider2D[] near = Physics2D.OverlapCircleAll(transform.position, weaponConfig.attackRadius);
        foreach (var collider in near) {
            if (energy < weaponConfig.attackCost) {
                return;
            }

            Cell obj = collider.GetComponentInParent<Cell>();
            if (obj == null || obj == this) {
                continue;
            }

            var stolen = obj.energy * weaponConfig.drainRate;
            energy += stolen;
            obj.energy -= stolen;

            energy -= weaponConfig.attackCost;
        }
    }
}

