using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Bounds : MonoBehaviour
{
    MinMaxF boundsX; 
    MinMaxF boundsY; 

    BoundaryParams boundaryConfig;

    void Awake()
    {
        boundaryConfig = GameObject.Find("Settings").GetComponent<Settings>().boundaryParams;

        var left   = transform.Find("left");
        var right  = transform.Find("right");
        var bottom = transform.Find("bottom");
        var top    = transform.Find("top");

        left.transform.Translate(-boundaryConfig.width/2, 0, 0);
        right.transform.Translate(boundaryConfig.width/2, 0, 0);
        bottom.transform.Translate(0, -boundaryConfig.height/2, 0);
        top.transform.Translate(0, boundaryConfig.height/2, 0);
        
        left.transform.localScale = new Vector3(boundaryConfig.thickness, boundaryConfig.height, 0);
        right.transform.localScale = new Vector3(boundaryConfig.thickness, boundaryConfig.height, 0);
        bottom.transform.localScale = new Vector3(boundaryConfig.width, boundaryConfig.thickness, 0);
        top.transform.localScale = new Vector3(boundaryConfig.width, boundaryConfig.thickness, 0);

        boundsX.min = -boundaryConfig.width/2 + boundaryConfig.thickness/2;
        boundsX.max =  boundaryConfig.width/2 - boundaryConfig.thickness/2;
        boundsY.min = -boundaryConfig.height/2 + boundaryConfig.thickness/2;
        boundsY.max =  boundaryConfig.height/2 - boundaryConfig.thickness/2;
    }

    public Vector3 GetRandomPos()
    {
        return new Vector3(boundsX.sample(), boundsY.sample(), 0.0f);
    }
}
