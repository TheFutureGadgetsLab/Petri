using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Bounds : MonoBehaviour
{
    MinMaxF boundsX; 
    MinMaxF boundsY; 

    void Awake()
    {
        var left   = transform.Find("left");
        var right  = transform.Find("right");
        var bottom = transform.Find("bottom");
        var top    = transform.Find("top");

        left.transform.Translate(-Settings.inst.boundary.width/2, 0, 0);
        right.transform.Translate(Settings.inst.boundary.width/2, 0, 0);
        bottom.transform.Translate(0, -Settings.inst.boundary.height/2, 0);
        top.transform.Translate(0, Settings.inst.boundary.height/2, 0);
        
        left.transform.localScale = new Vector3(Settings.inst.boundary.thickness, Settings.inst.boundary.height, 0);
        right.transform.localScale = new Vector3(Settings.inst.boundary.thickness, Settings.inst.boundary.height, 0);
        bottom.transform.localScale = new Vector3(Settings.inst.boundary.width, Settings.inst.boundary.thickness, 0);
        top.transform.localScale = new Vector3(Settings.inst.boundary.width, Settings.inst.boundary.thickness, 0);

        boundsX.min = -Settings.inst.boundary.width/2 + Settings.inst.boundary.thickness/2;
        boundsX.max =  Settings.inst.boundary.width/2 - Settings.inst.boundary.thickness/2;
        boundsY.min = -Settings.inst.boundary.height/2 + Settings.inst.boundary.thickness/2;
        boundsY.max =  Settings.inst.boundary.height/2 - Settings.inst.boundary.thickness/2;
    }

    public Vector3 GetRandomPos()
    {
        return new Vector3(boundsX.sample(), boundsY.sample(), 0.0f);
    }
}
