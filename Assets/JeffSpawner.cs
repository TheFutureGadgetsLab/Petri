using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class JeffSpawner : MonoBehaviour
{
    GameObject Jeff;
    public uint nJeffs;
    public Vector2 jeffScale = new Vector2(1, 1);

    // Start is called before the first frame update
    void Start()
    {
        Jeff = Resources.Load<GameObject>("Jeff");

        for (int i = 0; i < nJeffs; i++)
        {
            var jeff = GameObject.Instantiate(Jeff, new Vector3(Random.Range(-9f, 9f), Random.Range(-4f, 4f), 0), Quaternion.identity);
            jeff.transform.localScale = jeffScale;
        }
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
