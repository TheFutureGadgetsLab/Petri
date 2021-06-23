using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Propulsion : Cell
{
    PropulsionParams config;

    ConstantForce2D prop;
    Vector2 force;
    float torque;

    new protected void Start() {
        base.Start();

        config = GameObject.Find("Settings").GetComponent<Settings>().propulsionParams;

        prop = GetComponent<ConstantForce2D>();
        force = new Vector2(config.force.sample(), config.force.sample());
        torque = config.torque.sample();
    }

    private void FixedUpdate() {
        if (food > config.cost) {
            food -= config.cost;
            prop.relativeForce = force;
            prop.torque = torque;
        } else {
            prop.relativeForce = Vector2.zero;
            prop.torque = 0.0f;
        }
    }
}

