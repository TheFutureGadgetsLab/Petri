using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Propulsion : Cell
{
    PropulsionParams config;

    new protected void Start() {
        base.Start();

        config = GameObject.Find("Settings").GetComponent<Settings>().propulsionParams;

        // Set a random rotation
        transform.Rotate(Vector3.forward * Random.Range(-180, 180));

        var constantForce = GetComponent<ConstantForce2D>();
        constantForce.relativeForce = new Vector2(config.force.sample(), config.force.sample());

        constantForce.torque = config.torque.sample();
    }
}

