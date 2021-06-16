using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class JeffDoBeGoing : MonoBehaviour
{

    // Start is called before the first frame update
    void Start()
    {
        var force = new Vector2(Random.Range(-500f, 500f), Random.Range(-500f, 500f));

        var rigbod = GetComponent<Rigidbody2D>();
        rigbod.AddForce(force);
    }

    // Update is called once per frame
    void FixedUpdate()
    {
        var line = GetComponent<LineRenderer>();
        var spring = GetComponent<SpringJoint2D>();
        if (line == null) {
            return;
        }
        var body = spring.attachedRigidbody;

        line.SetPosition(0, spring.connectedBody.position);
        line.SetPosition(1, GetComponent<Transform>().position);
    }
    
}
