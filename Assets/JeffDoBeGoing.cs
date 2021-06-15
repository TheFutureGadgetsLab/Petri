using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class JeffDoBeGoing : MonoBehaviour
{

    // Start is called before the first frame update
    void Start()
    {
        var force = new Vector2(Random.Range(-100f, 100f), Random.Range(-100f, 100f));

        var rigbod = GetComponent<Rigidbody2D>();
        rigbod.AddForce(force);
    }

    // Update is called once per frame
    void FixedUpdate()
    {
    }
    
}
