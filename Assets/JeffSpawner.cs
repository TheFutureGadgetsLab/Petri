using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class JeffSpawner : MonoBehaviour
{
    GameObject Jeff;
    public uint nJeffs;

    // Start is called before the first frame update
    void Start()
    {
        Jeff = Resources.Load<GameObject>("Jeff");

        for (int i = 0; i < nJeffs; i++)
        {
            GameObject.Instantiate(Jeff, new Vector3(Random.Range(-9f, 9f), Random.Range(-4f, 4f), 0), Quaternion.identity);
        }
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
